
import { invoke } from "@tauri-apps/api/core";
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { Settings } from "../types";

export const load: PageLoad = async () => {
	try {
		const settings: Settings = await invoke("grab_settings"); // first_message (in rust) = firstMessage (in JS) :(
		return {settings: settings};
	} catch (err) {
		error(404, err as string);
	}
};
