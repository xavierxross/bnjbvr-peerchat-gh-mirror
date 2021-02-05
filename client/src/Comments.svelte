<script lang="ts">
    import {
        afterUpdate,
        beforeUpdate,
        onDestroy,
        onMount,
        tick,
    } from "svelte";
    import toast from "./toast";
    import { nickname as nicknameStore } from "./stores";
    import * as backend from "./backend";
    import NicknameField from "./NicknameField.svelte";

    // Retrieved from the page URL. Indicates what's the room identifier, used
    // to fetch the URL if we're landing here from the room URL.
    export let roomId: number | null = null;

    let messages: Message[] = [];
    let commentList;
    let ws;
    let autoscroll: boolean = false;
    let formComment = "";
    let formCommentBusy = false;
    let nickname = "";

    interface Message {
        time: number;
        author: string;
        content: string;
    }

    function debug(content: string) {
        toast.error(
            "Connexion au chat perdue: " + event.code + ": " + event.reason
        );
        onNewMessage({
            data: JSON.stringify({
                time: Date.now(),
                author: "PEERCHAT STATUS",
                content,
            }),
        });
    }

    onMount(() => {
        let onClose = function (event) {
            debug(
                "Connexion au chat perdue: " + event.code + ": " + event.reason
            );
        };
        ws = backend.chatWebsocket(roomId);
        ws.addEventListener("message", onNewMessage);
        ws.addEventListener("close", onClose);
        ws.addEventListener("error", (err) => {
            debug("Connexion au chat perdue: " + err.toString());
        });
        return () => {
            ws.removeEventListener("close", onClose);
            ws.close();
        };
    });

    function onNewMessage(event) {
        try {
            if (event.data instanceof Blob && event.data.size === 0) {
                // An empty ping sent by the server; ignore.
                return;
            }

            let msg = JSON.parse(event.data);
            messages = [...messages, msg];
            // Messages could come in unsorted order.
            // TODO consider requiring the server to send them in sorted order.
            messages.sort((a: Message, b: Message) => {
                let at = a.time;
                let bt = b.time;
                if (at < bt) return -1;
                if (at > bt) return 1;
                return 0;
            });
        } catch (err) {
            toast.error("Contenu malformé sur la ws: " + err.toString());
        }
    }

    const TIME_FORMAT = new Intl.DateTimeFormat("fr", {
        timeStyle: "medium",
    });
    function displayTime(timestamp: number) {
        return TIME_FORMAT.format(new Date(timestamp));
    }

    const AUTHOR_COLORS: Record<string, string> = {};
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
            commentList &&
            commentList.offsetHeight + commentList.scrollTop >
                commentList.scrollHeight - 20;
    });

    afterUpdate(() => {
        if (autoscroll) commentList.scrollTo(0, commentList.scrollHeight);
    });

    function sendMessageToServer(author: string, content: string) {
        ws.send(
            JSON.stringify({
                author,
                content,
            })
        );
    }

    async function sendMessage() {
        let comment = formComment.trim();
        if (comment.length === 0) {
            return;
        }

        formCommentBusy = true;
        try {
            sendMessageToServer(nickname, comment);
            formComment = "";
        } catch (err) {
            toast.error("Couldn't send message: " + err.toString());
        } finally {
            formCommentBusy = false;
        }
    }

    let previousNick: string | null = null;
    const unsubscribeNickname = nicknameStore.subscribe((value) => {
        nickname = value;
        if (previousNick !== null) {
            for (let m of messages) {
                if (m.author === previousNick) {
                    m.author = value;
                }
            }
            messages = messages;
            AUTHOR_COLORS[value] = AUTHOR_COLORS[previousNick];
            delete AUTHOR_COLORS[previousNick];
        }
        previousNick = value;
    });

    onDestroy(unsubscribeNickname);
</script>

<div class="comments-container">
    <ul class="comments" bind:this={commentList}>
        {#each messages as msg}
            <li>
                <strong>{displayTime(msg.time)}</strong>
                <span style={authorStyle(msg.author)}>{msg.author}</span>: {msg.content}
            </li>
        {/each}
    </ul>

    <form class="form" on:submit|preventDefault={sendMessage}>
        <NicknameField />
        <input
            type="text"
            class={formCommentBusy ? "waiting" : ""}
            disabled={formCommentBusy}
            bind:value={formComment}
            on:keydown={(event) => {
                if (event.key === "Enter") {
                    sendMessage();
                }
            }}
        />
    </form>
</div>

<style>
    .comments-container {
        height: 100%;
    }

    .comments {
        min-width: 100%;
        height: calc(100% - 3em);
        overflow-y: scroll;
        padding-left: 0;
        margin: 0;
    }

    .comments li {
        list-style-type: none;
    }

    .form {
        height: 3em;
        display: flex;
    }

    .form :global(.nickname-field) {
        max-width: 10em;
    }

    .waiting {
        opacity: 0.8;
    }
</style>
