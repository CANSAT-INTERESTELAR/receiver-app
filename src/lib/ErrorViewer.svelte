<script>
    import { listen } from '@tauri-apps/api/event'
    import { onMount } from 'svelte';
    import SlideTransition from './SlideTransition.svelte';
	import { Notification } from '@svelteuidev/core';
	import { Cross2 } from 'radix-icons-svelte';

    let errorsToShow = [];
    let showingError = false;

    onMount(async () => {
        await listen('error', (event) => {
            errorsToShow.push(event.payload.message);
            errorsToShow = errorsToShow; // This updates the value for Svelte
            showingError = true;
        })
    })

    function removeError() {
        errorsToShow.shift();
        errorsToShow = errorsToShow; // This updates the value for Svelte

        if (errorsToShow.length > 0) {
            showingError = true;
        } else {
            showingError = false;
        }
    }
</script>

<div class="fix-bottom-right">
    {#if showingError}
        <SlideTransition>
            <Notification icon={Cross2} color='red' on:close={removeError}>
                <div class="error-notification">
                    {errorsToShow[0]}
                </div>
            </Notification>
        </SlideTransition>
    {/if}
</div>

<style>
    .fix-bottom-right {
        position: fixed;
        bottom: 0;
        right: 0;

        max-width: 32rem;
        padding: 1rem;
    }

    .error-notification {
        word-wrap: break-word;
    }
</style>