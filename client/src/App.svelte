<script lang="ts">
    import { Router, Link, Route, createHistory } from "svelte-navigator";
    import { SvelteToast } from "@zerodevx/svelte-toast";

    import Home from "./Home.svelte";
    import Room from "./Room.svelte";

    import createHashSource from "./hashHistory";

    const hash = createHistory(createHashSource());
</script>

<Router history={hash}>
    <nav>
        <Link to="">Home</Link>
        <Link to="about">About</Link>
    </nav>

    <hr />

    <Route path="room/:roomId" component={Room} />
    <Route component={Home} />
</Router>

<SvelteToast />
