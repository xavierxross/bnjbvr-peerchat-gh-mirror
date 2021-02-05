<script lang="ts">
    import { onDestroy } from "svelte";
    import { nickname as nicknameStore } from "./stores";

    function updateNick() {
        let newNick = nickname.trim();
        if (prevNickname !== newNick) {
            nicknameStore.set(newNick);
        }
    }

    let nickname: string = "";
    let prevNickname: string | null = null;
    let unsubNickname = nicknameStore.subscribe((value) => {
        nickname = value;
        prevNickname = value;
    });

    onDestroy(() => {
        unsubNickname();
    });
</script>

<input
    class="nickname-field"
    type="text"
    bind:value={nickname}
    on:blur={updateNick}
    on:keydown={(key) => {
        if (key.code === "Enter") {
            updateNick();
        }
    }}
/>
