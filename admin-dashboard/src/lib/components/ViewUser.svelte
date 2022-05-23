<script lang="ts">
    import {loggedIn} from "../stores";
    import Spinner from "./Spinner.svelte";
    import type {PrivateUser} from "../../app";

    export let user_id: string
    export let screenSelected: string;
    let userChangeData: {
        username: string,
        email: string,
        admin: boolean,
        scopes: Array<string>
    }

    let user_initial: PrivateUser | null

    const getUserFunction = async (): Promise<null | PrivateUser> => {
        const res = await fetch(`/api/v1/admin/user?id=${user_id}`)
        if (res.status === 401) {
            loggedIn.set({status: false, admin: false})
            return
        }
        if (res.status === 200) {
            const json = await res.json()
            userChangeData = {
                username: json.username,
                admin: json.admin,
                email: json.email,
                scopes: json.scopes
            }
            user_initial = json
            return json
        }

    }

    const saveData = async () => {
        let data_to_submit = {
            id: user_id,
            username: userChangeData.username === user_initial.username ? undefined : userChangeData.username,
            admin: userChangeData.admin === user_initial.admin ? undefined : userChangeData.admin,
            email: userChangeData.email === user_initial.email ? undefined : userChangeData.email,
            scopes: JSON.stringify(userChangeData.scopes) === JSON.stringify(user_initial.scopes) ? undefined : userChangeData.scopes
        }
        const res = await fetch("/api/v1/admin/user", {
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
            getUser = getUserFunction()
        }
    }
    let tempScopeAdd = ""
    let getUser = getUserFunction()
    const deleteUser = async () => {
        if (!confirm("Are you sure you want to delete this user?!")) {
            return
        }
        await fetch(`/api/v1/admin/user?id=${user_id}`, {
            method: "DELETE"
        })
        screenSelected = "home"
    }

</script>
<button on:click={() => {screenSelected = "home"}}>Close</button>
<br>
<button on:click={deleteUser}>DELETE USER</button>

{#await getUser}
    <Spinner/>

{:then user}
    <div>
        <div>
            <label for="username">Username: </label>
            <input id="username" type="text" bind:value={userChangeData.username}>
        </div>
        <div>
            <label for="email">Email: </label>
            <input id="email" type="email" bind:value={userChangeData.email}>
        </div>
        <div>
            <label for="admin">Admin: </label>
            <input id="admin" type="checkbox" bind:checked={userChangeData.admin}>
        </div>
        <div>
            <div>
                <label for="addScope">Add scope:</label>
                <input id="addScope" type="text" bind:value={tempScopeAdd}>
                <button on:click={() => {userChangeData.scopes = [...userChangeData.scopes, tempScopeAdd]}}>Add scope
                </button>
            </div>
            <div>
                <ul>
                    {#each userChangeData.scopes as scope}
                        <li>{scope}
                            <button class="bg-red-700"
                                    on:click={() => {userChangeData.scopes = userChangeData.scopes.filter(m => m !== scope)}}>
                                Remove Scope
                            </button>
                        </li>
                    {/each}
                </ul>
            </div>
        </div>
    </div>
    <button on:click={saveData}>Save</button>

{/await}
