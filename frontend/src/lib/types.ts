export type UserRole = 'ADMIN' | 'MEMBER';

export interface UserResponse {
	id: string;
	name: string;
	email: string;
	role: UserRole;
	created_at: string;
	updated_at: string;
}

export interface TokenResponse {
	access_token: string;
	refresh_token: string;
}

export interface LoginResponse {
	user: UserResponse;
	tokens: TokenResponse;
}
