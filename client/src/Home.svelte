<script lang="ts">
    import { useNavigate } from "svelte-navigator";
    import { peertubeUrl } from "./stores";
    import { getRoomId } from "./room-url";
    import NicknameField from './NicknameField.svelte';

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

    <p>
    Bienvenu sur Peerchat ! Ce site vous permet d'ajouter un chat en temps réel
    à n'importe quelle vidéo Peertube. Pour cela, vous pouvez entrer l'URL
    (l'adresse à prendre de la barre d'adresse) dans le champ ci-dessous, vous
    choisir un pseudo, et commencer à papoter :-)
    </p>

    <div class="line">
        <label
            >URL Peertube :
            <input type="text" bind:value={$peertubeUrl} />
        </label>
    </div>

    <div class="line">
        <label>Votre surnom :
            <NicknameField />
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
