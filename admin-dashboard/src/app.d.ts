export interface PrivateUser {
	id: string;
	username: string;
	profile_pic: string;
	email: string;
	created_at: string;
	admin: boolean;
	scopes: string[];
	totp_enabled: boolean;
}

export interface App {
	id: string;
	name: string;
	description?: string;
	owner: string;
	created_at: string;
	updated_at: string;
	token_lifetime: number;
	domains: string[];
	enforce_totp: boolean;
}
