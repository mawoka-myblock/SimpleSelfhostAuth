import { writable } from 'svelte/store';

export const loggedIn = writable({ status: false, admin: false });
