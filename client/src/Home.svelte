<script lang="ts">
    import { useNavigate } from "svelte-navigator";
    import { peertubeUrl } from "./stores";
    import { getRoomId } from "./room-url";
    import NicknameField from "./NicknameField.svelte";
    import { _ } from "svelte-i18n";

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
    <h1>Peerchat</h1>

    <p>{$_("home.welcome")}</p>

    <div class="line">
        <label
            >{$_("home.peertube_url")}
            <input type="text" bind:value={$peertubeUrl} />
        </label>
    </div>

    <div class="line">
        <label>{$_("home.nickname")} <NicknameField /> </label>
    </div>

    <div class="line">
        <input
            type="submit"
            value={$_('home.start-button')}
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
