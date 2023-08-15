import { writable } from 'svelte/store';

export const latestSatRX = writable(JSON.parse("{}"));
export const totalRX = writable(JSON.parse("{\"data\":{}}"));
