<script lang="ts">
    import "./app.css"
    import {loggedIn} from "./lib/stores";
    import Spinner from "./lib/components/Spinner.svelte";
</script>

{#if $loggedIn.status}
    {#if $loggedIn.admin}
        {#await import("./lib/components/AdminDashboard.svelte")}
            <Spinner/>

        {:then c}
            <svelte:component this={c.default}/>
        {/await}
    {:else}
        {#await import("./lib/components/UserDashboard.svelte")}
            <Spinner/>

        {:then c}
            <svelte:component this={c.default}/>
        {/await}
    {/if}
{:else}
    {#await import("./lib/components/Login.svelte")}
        <Spinner/>

    {:then c}
        <svelte:component this={c.default}/>
    {/await}
{/if}



