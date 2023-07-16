<!-- https://dev.to/giorgosk/smooth-page-transitions-in-layout-svelte-with-sveltekit-or-sapper-4mm1 -->
<script>
    import { fly } from "svelte/transition";
</script>
  
<div
  in:fly="{{ y: -50, duration: 250, delay: 300 }}"
  out:fly="{{ y: -50, duration: 250 }}">
  <slot />
</div>