<script lang="ts">
    import {loggedIn} from "../stores";
    import Spinner from "./Spinner.svelte";

    const loadUser = async () => {
        const res = await fetch("/api/v1/users/me")
        if (res.status === 401) {
            loggedIn.set({status: false, admin: false})
        } else if (res.status === 200) {
            return await res.json()
        } else {
            console.error("ERROR!!!!!")
        }
    }

    let totpData: {
        url: string | null,
        qr_code: string | null,
        secret: string | null
    } = {
        url: null,
        qr_code: null,
        secret: null
    }
    const enableTOTP = async () => {
        if (!confirm("Are you sure you want to enable TOTP?!")) {
            return
        }
        const res = await fetch("/api/v1/users/setup_totp", {method: "POST"})
        if (res.status === 401) {
            loggedIn.set({status: false, admin: false})
            return
        } else if (res.status === 200) {
            let json = await res.json()
            totpData = {
                secret: json.secret,
                qr_code: json.qr_code,
                url: json.url
            }
        }
    }
</script>

{#await loadUser()}
    <Spinner/>

{:then user}
    <p>Username: {user.username}</p>
    <p>Email: {user.email}</p>
    <ul>Scopes:
        {#each user.scopes as scope}
            <li>{scope}</li>
        {/each}
    </ul>

    <button on:click={enableTOTP}>Enable TOTP</button>

{/await}

{#if totpData.qr_code}
    <div class="relative top-0 z-10">
        <img src="data:image/png;charset=utf-8;base64, {totpData.qr_code}" alt="QR-Code for TOTP-App">
        <a href={totpData.url}>Or click this link</a>
    </div>


{/if}