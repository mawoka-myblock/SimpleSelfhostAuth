<script lang="ts">
	import { loggedIn } from '../stores';
	import Spinner from './Spinner.svelte';

	const loadUser = async () => {
		const res = await fetch('/api/v1/users/me');
		if (res.status === 401) {
			loggedIn.set({ status: false, admin: false });
		} else if (res.status === 200) {
			return await res.json();
		} else {
			console.error('ERROR!!!!!');
		}
	};
	let user_data = loadUser();

	let totpData: {
		url: string | null;
		qr_code: string | null;
		secret: string | null;
	} = {
		url: null,
		qr_code: null,
		secret: null
	};
	const enableTOTP = async () => {
		if (!confirm('Are you sure you want to enable TOTP?!')) {
			return;
		}
		const res = await fetch('/api/v1/users/setup_totp', { method: 'POST' });
		if (res.status === 401) {
			loggedIn.set({ status: false, admin: false });
			return;
		} else if (res.status === 200) {
			let json = await res.json();
			totpData = {
				secret: json.secret,
				qr_code: json.qr_code,
				url: json.url
			};
		}
		user_data = loadUser();
	};

	const deactivateTOTP = async () => {
		if (!confirm('Are you sure you want to disable TOTP?!')) {
			return;
		}
		const totp_token = prompt('Please enter your totp-token');
		await fetch(`/api/v1/users/totp?totp=${totp_token}`, {
			method: 'DELETE'
		});
		await fetch('/api/v1/users/logout');
		loggedIn.set({ admin: false, status: false });
		user_data = loadUser();
	};
</script>

{#await user_data}
	<Spinner />
{:then user}
	<p>Username: {user.username}</p>
	<p>Email: {user.email}</p>
	{#if user.scopes.length === 0}
		<p>You don't have any scopes!</p>
	{:else}
		<ul>
			Scopes:
			{#each user.scopes as scope}
				<li>{scope}</li>
			{/each}
		</ul>
	{/if}
	{#if user.totp_enabled}
		<button on:click={deactivateTOTP}>Disable TOTP</button>
	{:else}
		<button on:click={enableTOTP}>Enable TOTP</button>
	{/if}
{/await}

{#if totpData.qr_code}
	<div class="relative top-0 z-10">
		<img
			src="data:image/png;charset=utf-8;base64, {totpData.qr_code}"
			alt="QR-Code for TOTP-App"
		/>
		<a href={totpData.url}>Or click this link</a>
	</div>
{/if}
