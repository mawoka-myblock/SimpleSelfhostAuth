<script lang="ts">
	import './app.css';
	import { loggedIn } from './lib/stores';
	import Spinner from './lib/components/Spinner.svelte';
	import Tabs from './lib/components/Tabs.svelte';

	const CheckIfLoggedIn = async () => {
		const res = await fetch('/api/v1/users/me');
		if (res.status === 401) {
			loggedIn.set({ status: false, admin: false });
		} else if (res.status === 200) {
			let json = await res.json();
			loggedIn.set({ status: true, admin: json.admin });
		}
	};
	let tabSelected = 'Home';
</script>

{#await CheckIfLoggedIn()}
	<Spinner />
{:then _}
	{#if $loggedIn.status}
		{#if $loggedIn.admin}
			<Tabs available_tabs={['Home', 'Admin']} bind:selected_tab={tabSelected} />

			{#if tabSelected === 'Admin'}
				{#await import('./lib/components/AdminDashboard.svelte')}
					<Spinner />
				{:then c}
					<svelte:component this={c.default} />
				{/await}
			{:else}
				{#await import('./lib/components/UserDashboard.svelte')}
					<Spinner />
				{:then c}
					<svelte:component this={c.default} />
				{/await}
			{/if}
		{:else}
			{#await import('./lib/components/UserDashboard.svelte')}
				<Spinner />
			{:then c}
				<svelte:component this={c.default} />
			{/await}
		{/if}
	{:else}
		{#await import('./lib/components/Login.svelte')}
			<Spinner />
		{:then c}
			<svelte:component this={c.default} />
		{/await}
	{/if}
{/await}
