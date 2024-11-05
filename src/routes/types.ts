
export type Character = {
		id: string,
		name: string,
		description: string,
		system_message: string,
		first_message: string,
		conversations: string[],
}

export type Message = { person: "system" | "user" | "assistant", content: string };
export type CharacterConversation = {
    id: string,
    from_char_id: string,
    memory_id: string,
    log: Message[],
}
export type MessageRequest = {
		memory_id: string,
		memories: Message[],
		log: Message[],
		regen: boolean,
}
export type Settings = {
    id: string,
    link: string,
    temp: number,
    top_p: number,
    top_k: number,
    min_p: number,
    typical_p: number,
    repeat_penalty: number,
    tfs_z: number,
    mirostat_mode: number,
    mirostat_tau: number,
    mirostat_eta: number,
}



