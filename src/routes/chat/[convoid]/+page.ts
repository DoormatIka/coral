
import { invoke } from "@tauri-apps/api/core";
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type {Character, CharacterConversation} from "../../types";

export const load: PageLoad = async ({ params }) => {
	if (params.convoid.length <= 0) {
		error(404, 'Not found');
	}
	try {
		const conversation: CharacterConversation = await invoke("grab_conversation", {"chatId": params.convoid}); // first_message (in rust) = firstMessage (in JS) :(
		const character: Character = await invoke("grab_character", {"id": conversation.from_char_id}); // first_message (in rust) = firstMessage (in JS) :(
		return {character: character, conversation: conversation};
	} catch (err) {
		error(404, err as string);
	}
};
