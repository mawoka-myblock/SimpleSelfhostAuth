<script lang="ts">
    import {loggedIn} from "../stores";
    import type {PrivateUser, App} from "../../app";
    import Spinner from "./Spinner.svelte";
    import {createTippy} from 'svelte-tippy';
    import 'tippy.js/animations/perspective-subtle.css';
    import 'tippy.js/dist/tippy.css';

    const tippy = createTippy({
        arrow: true,
        animation: 'perspective-subtle',
        placement: 'right'
    });
    let appScreenSelected = false

    const getUsers = async (): Promise<Array<PrivateUser>> => {
        const res = await fetch("/api/v1/admin/users?offset=0")
        if (res.status === 200) {
            return await res.json()
        } else if (res.status === 401) {
            loggedIn.set({status: false, admin: false})
        }
    }

    const getAppsFunction = async (): Promise<Array<App>> => {
        const res = await fetch("/api/v1/admin/apps?offset=0")
        if (res.status === 200) {
            return await res.json()
        } else if (res.status === 401) {
            loggedIn.set({status: false, admin: false})
        }
    }
    let getUsersVar = getUsers();
    let getApps = getAppsFunction()


    const addScope = async (user_id: string, existing_scopes: Array<string>) => {
        const scope = prompt("Enter the scope you want to add!")
        existing_scopes.push(scope)
        const res = await fetch("/api/v1/admin/user", {
            method: "PATCH",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                id: user_id,
                scopes: existing_scopes
            })
        })
        getUsersVar = getUsers()
    }
    const addDomain = async (app_id: string, existing_domains: Array<string>) => {
        const domain = prompt("Enter the domain you want to add!")
        existing_domains.push(domain)
        const res = await fetch("/api/v1/admin/app", {
            method: "PATCH",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                id: app_id,
                scopes: existing_domains
            })
        })
        getApps = getAppsFunction()
    }
    let screenSelected: string = "home"
    let selectedUserId = ""
    let selectedAppId = ""
</script>

<div class="top-0 left-0 w-screen grid grid-cols-2">
    <button on:click={() => {appScreenSelected = true}} class:bg-green-500={appScreenSelected}>
        Apps
    </button>
    <button on:click={() => {appScreenSelected = false}} class:bg-green-500={!appScreenSelected}>
        Users
    </button>
</div>

{#if appScreenSelected}
    <!-- Apps -->
    {#if screenSelected === "home"}
        {#await getApps}
            <Spinner/>
        {:then apps}
            <button type="button"
                    on:click={() => {screenSelected = "createApp"}}
                    class="py-2.5 px-5 mr-2 mb-2 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700">
                Create App
            </button>
            <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                    <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                    <tr>
                        <th scope="col" class="px-6 py-3">
                            Name
                        </th>
                        <th scope="col" class="px-6 py-3">
                            ID
                        </th>
                        <th scope="col" class="px-6 py-3">
                            Created At
                        </th>
                        <th scope="col" class="px-6 py-3">
                            Domains
                        </th>
                        <!--<th scope="col" class="px-6 py-3">
                            <span class="sr-only">Edit</span>
                        </th>-->
                    </tr>
                    </thead>
                    <tbody>
                    {#each apps as app}
                        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 hover:cursor-pointer"
                            on:click={() => {selectedAppId = app.id; screenSelected = "viewApp"}}>
                            <th scope="row"
                                class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap">
                                {app.name}
                            </th>
                            <td class="px-6 py-4">
                                {app.id}
                            </td>
                            <td class="px-6 py-4">
                                {app.created_at}
                            </td>
                            <td class="px-6 py-4"
                                use:tippy={{content: app.domains.length !== 0 ? app.domains.join(", ") : "No domains"}}>
                                Domains [{app.domains.length}]
                                <button on:click={() => {addDomain(app.id, app.domains)}}>Add scope</button>
                            </td>
                        </tr>
                    {/each}

                    </tbody>
                </table>
            </div>
        {/await}
    {:else if screenSelected === "viewApp"}
        <div>
            {#await import("./ViewApp.svelte")}
                <Spinner/>
            {:then c}
                <svelte:component this={c.default} bind:screenSelected app_id={selectedAppId}></svelte:component>
            {/await}
        </div>
    {:else if screenSelected === "createApp"}
        <div>
            {#await import("./CreateAppScreen.svelte")}
                <Spinner/>
            {:then c}
                <svelte:component this={c.default} bind:screenSelected app_id={selectedAppId}></svelte:component>
            {/await}
        </div>

    {/if}

{:else}
    <!-- Users -->
    {#if screenSelected === "home"}
        {#await getUsersVar}
            <Spinner/>

        {:then users}
            <button type="button"
                    on:click={() => {screenSelected = "createUser"}}
                    class="py-2.5 px-5 mr-2 mb-2 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700">
                Create User
            </button>
            <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                    <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                    <tr>
                        <th scope="col" class="px-6 py-3">
                            Username
                        </th>
                        <th scope="col" class="px-6 py-3">
                            Email
                        </th>
                        <th scope="col" class="px-6 py-3">
                            Admin
                        </th>
                        <th scope="col" class="px-6 py-3">
                            Scopes
                        </th>
                        <!--<th scope="col" class="px-6 py-3">
                            <span class="sr-only">Edit</span>
                        </th>-->
                    </tr>
                    </thead>
                    <tbody>
                    {#each users as user}
                        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 hover:cursor-pointer"
                            on:click={() => {selectedUserId = user.id; screenSelected = "viewUser"}}>
                            <th scope="row"
                                class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap">
                                {user.username}
                            </th>
                            <td class="px-6 py-4">
                                {user.email}
                            </td>
                            <td class="px-6 py-4">
                                {#if user.admin}
                                    ✅
                                {:else}
                                    ❌
                                {/if}
                            </td>
                            <td class="px-6 py-4"
                                use:tippy={{content: user.scopes.length !== 0 ? user.scopes.join(", ") : "No scopes"}}>
                                Scopes [{user.scopes.length}]
                                <button on:click={() => {addScope(user.id, user.scopes)}}>Add scope</button>
                            </td>
                        </tr>
                    {/each}

                    </tbody>
                </table>
            </div>
        {/await}
    {:else if screenSelected === "createUser"}
        <div>
            {#await import("./CreateUserScreen.svelte")}
                <Spinner/>
            {:then c}
                <svelte:component this={c.default} bind:screenSelected></svelte:component>
            {/await}
        </div>
    {:else if screenSelected === "viewUser"}
        <div>
            {#await import("./ViewUser.svelte")}
                <Spinner/>
            {:then c}
                <svelte:component this={c.default} bind:screenSelected user_id={selectedUserId}></svelte:component>
            {/await}
        </div>
    {/if}

{/if}
