
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
