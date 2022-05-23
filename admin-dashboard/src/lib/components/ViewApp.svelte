<script lang="ts">
    import {loggedIn} from "../stores";
    import Spinner from "./Spinner.svelte";
    import type {App} from "../../app";

    export let app_id: string
    export let screenSelected: string;
    let appChangeData: {
        name: string,
        description: string,
        token_lifetime: number,
        domains: Array<string>
    }

    let app_initial: App | null

    const getAppFunction = async (): Promise<null | App> => {
        const res = await fetch(`/api/v1/admin/app?id=${app_id}`)
        if (res.status === 401) {
            loggedIn.set({status: false, admin: false})
            return
        }
        if (res.status === 200) {
            const json = await res.json()
            appChangeData = {
                description: json.description,
                domains: json.domains,
                name: json.name,
                token_lifetime: json.token_lifetime
            }
            app_initial = json
            return json
        }

    }

    const saveData = async () => {
        let data_to_submit = {
            id: app_id,
            name: appChangeData.name === app_initial.name ? undefined : appChangeData.name,
            description: appChangeData.description === app_initial.description ? undefined : appChangeData.description,
            token_lifetime: appChangeData.token_lifetime === app_initial.token_lifetime ? undefined : appChangeData.token_lifetime,
            domains: JSON.stringify(appChangeData.domains) === JSON.stringify(app_initial.domains) ? undefined : appChangeData.domains
        }
        const res = await fetch("/api/v1/admin/app", {
            method: "PATCH",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(data_to_submit)
        })
        if (res.status === 401) {
            loggedIn.set({status: false, admin: false})
            return
        }
        if (res.status === 200) {
            getApp = getAppFunction()
        }
    }
    let tempDomainAdd = ""
    let getApp = getAppFunction()

    const deleteApp = async () => {
        if (!confirm("Are you sure you want to delete this app?!")) {
            return
        }
        await fetch(`/api/v1/admin/app?id=${app_id}`, {
            method: "DELETE"
        })
        screenSelected = "home"
    }

</script>
<button on:click={() => {screenSelected = "home"}}>Close</button>
<br>
<button on:click={deleteApp}>DELETE APP</button>

{#await getApp}
    <Spinner/>

{:then user}
    <div>
        <div>
            <label for="name">Appname: </label>
            <input id="name" type="text" bind:value={appChangeData.name}>
        </div>
        <div>
            <label for="description">Description: </label>
            <textarea id="description" type="text" bind:value={appChangeData.description} />
        </div>
        <div>
            <label for="token_lifetime">Token Lifetime: </label>
            <input id="token_lifetime" type="number" bind:value={appChangeData.token_lifetime}>
        </div>
        <div>
            <div>
                <label for="addDomain">Add domain:</label>
                <input id="addDomain" type="text" bind:value={tempDomainAdd}>
                <button on:click={() => {appChangeData.domains = [...appChangeData.domains, tempDomainAdd]}}>Add domain
                </button>
            </div>
            <div>
                <ul>
                    {#each appChangeData.domains as domain}
                        <li>{domain}
                            <button class="bg-red-700"
                                    on:click={() => {appChangeData.domains = appChangeData.domains.filter(m => m !== domain)}}>
                                Remove domain
                            </button>
                        </li>
                    {/each}
                </ul>
            </div>
        </div>
    </div>
    <button on:click={saveData}>Save</button>

{/await}
