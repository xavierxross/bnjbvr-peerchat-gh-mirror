<script lang="ts">
    import { afterUpdate, beforeUpdate, onMount } from "svelte";
    import { nickname } from './stores';

    export let roomId: number | null = null;

    interface Message {
        time: number;
        author: string;
        content: string;
    }

    // TODO stub messages
    let fakeMessages: Message[] = [
        {
            time: Date.now() - 5000,
            author: "Jean-Jacques",
            content: "C'est vrai !",
        },
        {
            time: Date.now() - 3000,
            author: "Marie-Yvonne",
            content: "Il a raison !!",
        },
        {
            time: Date.now() - 1000,
            author: "FosseSceptique",
            content: "Sources??",
        },
        {
            time: Date.now() - 500,
            author: "stonks",
            content: "buy $gme",
        },
        {
            time: Date.now() + 500,
            author: "Georges",
            content: "J'ai connu un mec de droite une fois, il avait dix fois plus de classe",
        },
        {
            time: Date.now() + 1000,
            author: "ChefIndien69",
            content: "Qu'est-ce que j'ai avoir avec Georges ? Rien en fait !",
        },
    ];

    function fetchMessages(): Promise<Message[]> {
        return new Promise((ok) => {
            setTimeout(() => {
                ok(fakeMessages);
            }, 1000);
        });
    }

    let stopFakeComments = false;

    function generateNewFake() {
        let randomId = (Math.random() * messages.length) | 0;
        let randomId2 = (Math.random() * messages.length) | 0;

        let randomMessage: Message = {
            time: Date.now(),
            author: messages[randomId].author,
            content: messages[randomId2].content,
        };

        onNewMessage(randomMessage);

        let nextMsgIn = Math.random() * 2000;
        if (!stopFakeComments) {
            setTimeout(generateNewFake, nextMsgIn);
        }
    }
    setTimeout(generateNewFake, 1500);
    // end of stub messages

    let messages = [];
    let list;

    onMount(async () => {
        messages = await fetchMessages();
    });

    function onNewMessage(message: Message) {
        messages = [...messages, message];
    }

    const timeFormat = new Intl.DateTimeFormat("fr", {
        timeStyle: "medium",
    });
    function displayTime(timestamp: number) {
        return timeFormat.format(new Date(timestamp));
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

    let autoscroll: boolean = false;
    beforeUpdate(() => {
        autoscroll =
            list && list.offsetHeight + list.scrollTop > list.scrollHeight - 20;
    });

    afterUpdate(() => {
        if (autoscroll) list.scrollTo(0, list.scrollHeight);
    });

    function sendMessageToServer(
        author: string,
        comment: string
    ): Promise<Message> {
        return new Promise((ok) =>
            setTimeout(
                () =>
                    ok({
                        time: Date.now(),
                        author,
                        content: comment,
                    }),
                1000
            )
        );
    }

    let formComment = "";
    let formStyle = "";

    async function sendMessage() {
        let comment = formComment;
        formComment = "";

        try {
            formStyle = "waiting";
            let msg = await sendMessageToServer($nickname, comment);
            formStyle = "";

            onNewMessage(msg);
        } catch (err) {
            formComment = comment;
        }
    }
</script>

<div class="comments-container">
    <ul class="comments" bind:this={list}>
        {#each messages as msg}
            <li>
                <strong>{displayTime(msg.time)}</strong>
                <span style={authorStyle(msg.author)}>{msg.author}</span>: {msg.content}
            </li>
        {/each}
    </ul>

    <form class="form" on:submit|preventDefault={sendMessage}>
        <button on:click={() => (stopFakeComments = true)}>Stop</button>

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
