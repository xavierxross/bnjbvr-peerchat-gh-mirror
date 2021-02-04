<script lang="ts">
    import { useNavigate } from "svelte-navigator";
    import { nickname, peertubeUrl } from "./stores";
    import { getRoomId } from "./room-url";

    const navigate = useNavigate();

    async function queryServerForRoom() {
        waitingMsg = "Réticulation des splines...";
        try {
            let id = await getRoomId($peertubeUrl);
            navigate("/room/" + id);
        } catch (err) {
            waitingMsg = `Erreur ! ${err.toString()}`;
        }
    }

    let waitingMsg: string | null = null;
</script>

<form class="form">
    <div class="line">
        <label
            >URL Peertube :
            <input type="text" bind:value={$peertubeUrl} />
        </label>
    </div>

    <div class="line">
        <label
            >Votre surnom :
            <input type="text" bind:value={$nickname} />
        </label>
    </div>

    <div class="line">
        <input
            type="submit"
            value="Go!"
            on:click|preventDefault={queryServerForRoom}
        />
    </div>

    {#if waitingMsg !== null}
        <div>{waitingMsg}</div>
    {/if}
</form>

<style>
    .form {
        width: 30em;
        margin-left: auto;
        margin-right: auto;
    }

    .line {
        margin-bottom: 1em;
    }
</style>
