<!-- https://dev.to/giorgosk/smooth-page-transitions-in-layout-svelte-with-sveltekit-or-sapper-4mm1 -->
<script>
    import { slide } from "svelte/transition";
</script>
  
<div
  in:slide="{{ y: -50, duration: 250, delay: 25 }}"
  out:slide="{{ y: -50, duration: 250 }}">
  <slot />
</div>