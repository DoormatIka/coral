

import { invoke } from "@tauri-apps/api/core";
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type {Character, CharacterConversation} from "../../types";

export const load: PageLoad = async ({ params }) => {
	if (params.charid.length <= 0) {
		error(404, 'Not found');
	}
	const char_details: Character = await invoke("grab_character", {id: params.charid}); // first_message (in rust) = firstMessage (in JS) :(
	const convo = char_details.conversations.at(0);

	let conversation: CharacterConversation;
	if (convo) {
		conversation = await invoke("grab_conversation", {id: convo});
	} else {
		conversation = await invoke("add_conversation", {charId: char_details.id});
	}
	return {character: char_details, conversation: conversation};
};
