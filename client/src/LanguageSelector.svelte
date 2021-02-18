<script lang="ts">
    import { locale } from 'svelte-i18n';
    import { supportedLanguages } from './i18n.ts';
</script>

<select bind:value={$locale}>
    {#each supportedLanguages as option}
        <option value={option[0]}>{option[1]}</option>
    {/each}
</select>
