<script lang="ts">
    import { afterUpdate, beforeUpdate, onMount } from "svelte";
    import { toast } from "@zerodevx/svelte-toast";
    import { nickname } from "./stores";
    import * as backend from "./backend";

    export let roomId: number | null = null;

    interface Message {
        time: number;
        author: string;
        content: string;
    }

    let messages = [];
    let commentsList;
    let ws;
    let autoscroll: boolean = false;
    let formComment = "";
    let formStyle = "";

    onMount(() => {
        ws = backend.chatWebsocket(roomId);
        ws.addEventListener("message", onNewWsEvent);
        ws.addEventListener("error", (err) => {
            toast.push("Connexion au chat perdue: " + err.toString());
        });
        return () => {
            ws.close();
        };
    });

    function onNewWsEvent(event) {
        try {
            let msg = JSON.parse(event.data);
            messages = [...messages, msg];
        } catch (err) {
            toast.push("Contenu malformé sur la ws: " + err.toString());
        }
    }

    const TIME_FORMAT = new Intl.DateTimeFormat("fr", {
        timeStyle: "medium",
    });
    function displayTime(timestamp: number) {
        return TIME_FORMAT.format(new Date(timestamp));
    }

    const AUTHOR_COLORS = {};
    const AVAILABLE_COLORS = ["#ff0000", "#00ff00", "#0000ff"];
    let nextColor = 0;
    function getNextColor() {
        if (nextColor >= AVAILABLE_COLORS.length) {
            nextColor = 0;
        }
        return AVAILABLE_COLORS[nextColor++];
    }
    function authorStyle(author: string): string {
        if (typeof AUTHOR_COLORS[author] !== "undefined") {
            return AUTHOR_COLORS[author];
        }
        let r: string;
        AUTHOR_COLORS[author] = r = `color: ${getNextColor()}`;
        return r;
    }

    beforeUpdate(() => {
        autoscroll =
            commentsList &&
            commentsList.offsetHeight + commentsList.scrollTop >
                commentsList.scrollHeight - 20;
    });

    afterUpdate(() => {
        if (autoscroll) commentsList.scrollTo(0, commentsList.scrollHeight);
    });

    function sendMessageToServer(author: string, content: string) {
        ws.send(
            JSON.stringify({
                author,
                content,
            })
        );
    }

    function sendMessage() {
        let comment = formComment;
        formComment = "";

        try {
            formStyle = "waiting";
            sendMessageToServer($nickname, comment);
            formStyle = "";
        } catch (err) {
            formComment = comment;
        }
    }
</script>

<div class="comments-container">
    <ul class="comments" bind:this={commentsList}>
        {#each messages as msg}
            <li>
                <strong>{displayTime(msg.time)}</strong>
                <span style={authorStyle(msg.author)}>{msg.author}</span>: {msg.content}
            </li>
        {/each}
    </ul>

    <form class="form" on:submit|preventDefault={sendMessage}>
        <input type="text" style={formStyle} bind:value={formComment} />
    </form>
</div>

<style>
    .comments-container {
        height: 100%;
    }

    .comments {
        min-width: 100%;
        height: 90%;
        overflow-y: scroll;
        padding-left: 0;
        margin: 0;
    }

    .comments li {
        list-style-type: none;
    }

    .form {
        height: 10%;
        display: flex;
    }

    .form input {
        flex: 1 1 80%;
    }

    .waiting {
        opacity: 0.8;
    }
</style>
