

import { invoke } from "@tauri-apps/api/core";
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type {Character, CharacterConversation} from "../../types";

export const load: PageLoad = async ({ params }) => {
	console.log(params)
	if (params.charid.length <= 0) {
		error(404, 'Not found');
	}
	try {
		const char_details: Character = await invoke("grab_character", {"id": params.charid}); // first_message (in rust) = firstMessage (in JS) :(
		const conversation_id = char_details.conversations.at(0);
		let conversation: CharacterConversation;
		if (conversation_id) {
			conversation = await invoke("grab_conversation", {chatId: conversation_id});
		} else {
			const conversation_id = await invoke("add_conversation", {charId: char_details.id});
			await invoke("update_character", {charId: char_details.id, conversations: [conversation_id]});
			conversation = await invoke("grab_conversation", {chatId: conversation_id});
			// havent set the character's conversation yet, use update for this.
		}
		return {character: char_details, conversation: conversation};
	} catch (err) {
		error(404, err as string);
	}
};
