<script lang="ts">
    import "./app.css"
    import {loggedIn} from "./lib/stores";
    import Spinner from "./lib/components/Spinner.svelte";

    let adminScreenSelected = false
    const CheckIfLoggedIn = async () => {
        const res = await fetch("/api/v1/users/me")
        if (res.status === 401) {
            loggedIn.set({status: false, admin: false})
        } else if (res.status === 200) {
            let json = await res.json()
            loggedIn.set({status: true, admin: json.admin})
        }
    }
</script>

{#await CheckIfLoggedIn()}
    <Spinner/>

{:then _}

    {#if $loggedIn.status}
        {#if $loggedIn.admin}
            <div class="top-0 left-0 w-screen grid grid-cols-2">
                <button on:click={() => {adminScreenSelected = true}} class:bg-green-500={adminScreenSelected}>
                    Admin-Screen
                </button>
                <button on:click={() => {adminScreenSelected = false}} class:bg-green-500={!adminScreenSelected}>
                    User-Screen
                </button>
            </div>
            {#if adminScreenSelected}
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


{/await}
