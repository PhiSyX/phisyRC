export type Room = {
	name: string;
	type: "channel" | "private" | "custom-window";
	messages: string[];
};

export type Server = {
	name: string;
	rooms: Room[];
};
