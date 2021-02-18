<script lang="ts">
    import { isLoading, _ } from "svelte-i18n";
    import { Router, Link, Route, createHistory } from "svelte-navigator";
    import { SvelteToast } from "@zerodevx/svelte-toast";

    import "./i18n.ts";

    import Home from "./Home.svelte";
    import Room from "./Room.svelte";
    import LanguageSelector from "./LanguageSelector.svelte";

    import createHashSource from "./hashHistory";

    const hash = createHistory(createHashSource());
</script>

<Router history={hash}>
    {#if $isLoading}
        Reticulating splines...
    {:else}
        <nav>
            <div>
                <Link to="/">{$_("home-link")}</Link> |
                <a href="https://framagit.org/bnjbvr/peerchat" rel="noopener noreferrer" target="_blank">{$_("source-link")}</a>
            </div>

            <div>
                Language: <LanguageSelector />
            </div>
        </nav>

        <hr />

        <Route path="/room/:roomId" component={Room} />
        <Route component={Home} />
    {/if}
</Router>

<SvelteToast />

<style>
    nav {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
</style>
