import { writable } from 'svelte/store';

export const latestRX = writable(0);
export const latestHeightByPressure = writable(0);