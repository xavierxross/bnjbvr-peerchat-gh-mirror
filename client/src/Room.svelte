<script lang="ts">
    import { onMount } from "svelte";
    import toast from './toast';

    import { getRoomUrl } from "./room-url.ts";
    import Comments from "./Comments.svelte";
import { _ } from "svelte-i18n";

    export let roomId: number;

    let videoTitle = "Video";
    let embedUrl = null;
    let urlField;

    onMount(async function onMount() {
        let urlString;
        try {
            urlString = await getRoomUrl(roomId);
        } catch (err) {
            toast.error(err.toString());
        }
        embedUrl = urlString.replace("/watch/", "/embed/");

        let url = new URL(urlString);
        let peertubeId = url.pathname.split("/").pop();
        let api = `https://${url.hostname}/api/v1/videos/${peertubeId}`;

        try {
            let result = await fetch(api);
            let json = await result.json();
            videoTitle = json.name;
        } catch (err) {
            console.error("unable to fetch title");
        }
    });

    function copyUrl() {
        if (!urlField) {
            return;
        }
        urlField.select();
        document.execCommand("copy");
        toast.info($_('room.copied'), { duration: 2000 });
    }
</script>

<h1>{videoTitle} (#{roomId})</h1>

<p>
    {$_('room.share-url')}
    <input
        type="text"
        bind:this={urlField}
        on:click={copyUrl}
        value={window.location.toString()}
    />
</p>

{#if embedUrl === null}
    Chargement de l'URL...
{:else}
    <div class="flex">
        <iframe
            class="video"
            title={videoTitle}
            sandbox="allow-same-origin allow-scripts allow-popups"
            src={embedUrl}
            frameborder="0"
            allowfullscreen
        />
        <Comments {roomId} />
    </div>
{/if}

<style>
    .flex {
        display: flex;
        height: 80%;
    }

    /* TODO: this doesn't work, why? */
    .flex > * {
        flex: 0 0;
    }

    :global(.comments-container) {
        flex: 0 0;
    }

    .video {
        flex: 1 1 80%;
    }
</style>
