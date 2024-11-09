
import { invoke } from "@tauri-apps/api/core";
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type {Character, CharacterConversation} from "../../../types";

export const load: PageLoad = async ({ params }) => {
	if (params.charid.length <= 0) {
		error(404, 'Not found');
	}
	try {
		const char_details: Character = await invoke("grab_character", {"id": params.charid}); // first_message (in rust) = firstMessage (in JS) :(
		const conversations: CharacterConversation[] = await invoke("grab_conversation_list", {"charId": params.charid});
		return {character: char_details, conversations: conversations};
	} catch (err) {
		error(404, err as string);
	}
};
