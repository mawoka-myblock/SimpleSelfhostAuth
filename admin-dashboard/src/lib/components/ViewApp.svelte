<script lang="ts">
	import { loggedIn } from '../stores';
	import Spinner from './Spinner.svelte';
	import type { App } from '../../app';
	import { clickOutside } from '../clickOutside';
	import Switch from './Switch.svelte';

	export let app_id: string;
	export let screenSelected: string;
	let appChangeData: {
		name: string;
		description: string;
		token_lifetime: number;
		domains: Array<string>;
		enforce_totp: boolean;
	};

	let app_initial: App | null;

	const getAppFunction = async (): Promise<null | App> => {
		const res = await fetch(`/api/v1/admin/app?id=${app_id}`);
		if (res.status === 401) {
			loggedIn.set({ status: false, admin: false });
			return;
		}
		if (res.status === 200) {
			const json = await res.json();
			appChangeData = {
				description: json.description,
				domains: json.domains,
				name: json.name,
				token_lifetime: json.token_lifetime,
				enforce_totp: json.enforce_totp
			};
			app_initial = json;
			return json;
		}
	};

	const saveData = async () => {
		let data_to_submit = {
			id: app_id,
			name: appChangeData.name === app_initial.name ? undefined : appChangeData.name,
			description:
				appChangeData.description === app_initial.description
					? undefined
					: appChangeData.description,
			token_lifetime:
				appChangeData.token_lifetime === app_initial.token_lifetime
					? undefined
					: appChangeData.token_lifetime,
			domains:
				JSON.stringify(appChangeData.domains) === JSON.stringify(app_initial.domains)
					? undefined
					: appChangeData.domains,
			enforce_totp:
				appChangeData.enforce_totp === app_initial.enforce_totp
					? undefined
					: appChangeData.enforce_totp
		};
		const res = await fetch('/api/v1/admin/app', {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(data_to_submit)
		});
		if (res.status === 401) {
			loggedIn.set({ status: false, admin: false });
			return;
		}
		if (res.status === 200) {
			getApp = getAppFunction();
		}
	};
	let tempDomainAdd = '';
	let getApp = getAppFunction();
	let delete_confirm = false;

	const deleteApp = async () => {
		if (!delete_confirm) {
			return;
		}
		await fetch(`/api/v1/admin/app?id=${app_id}`, {
			method: 'DELETE'
		});
		delete_confirm = false;
		screenSelected = 'home';
	};
</script>

<div class="flex w-screen">
	<button
		type="button"
		class="bg-white rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500 place-self-end"
		on:click={() => {
			screenSelected = 'home';
		}}
	>
		<span class="sr-only">Close menu</span>
		<!-- Heroicons: outline/x -->
		<svg
			class="h-6 w-6"
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
			aria-hidden="true"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M6 18L18 6M6 6l12 12"
			/>
		</svg>
	</button>
</div>

<button
	class="relative inline-block px-4 py-1 overflow-hidden border border-red-600 group focus:outline-none focus:ring rounded-md"
	on:click={() => {
		deleteApp().then();
		delete_confirm = true;
	}}
	use:clickOutside={{ enabled: delete_confirm, cb: () => (delete_confirm = false) }}
>
	<span
		class="absolute inset-y-0 left-0 w-[2px] transition-all bg-red-600 group-hover:w-full group-active:bg-red-500"
	/>

	<span
		class="relative text-sm font-medium text-red-600 transition-colors group-hover:text-white"
	>
		{#if delete_confirm}
			Confirm
		{:else}
			Delete App
		{/if}
	</span>
</button>

{#await getApp}
	<Spinner />
{:then user}
	<div class="pl-2 pb-4">
		<div>
			<label for="name">Appname: </label>
			<input
				id="name"
				type="text"
				bind:value={appChangeData.name}
				class="border-b border-dotted border-black my-2"
			/>
		</div>
		<div class="align-middle">
			<label for="description" class="align-middle">Description: </label>
			<textarea
				id="description"
				type="text"
				bind:value={appChangeData.description}
				class="border border-black border-dotted rounded-md py-2 align-middle"
			/>
		</div>
		<div>
			<label for="token_lifetime">Token Lifetime: </label>
			<input
				id="token_lifetime"
				type="number"
				bind:value={appChangeData.token_lifetime}
				class="border-b border-dotted border-black"
			/>
		</div>
		<div>
			<p>Enforce TOTP?</p>
			<Switch bind:on={appChangeData.enforce_totp} />
		</div>
		<div>
			<div>
				<label for="addDomain">Add domain:</label>
				<input
					id="addDomain"
					type="text"
					bind:value={tempDomainAdd}
					class="border-b border-dotted border-black"
				/>
				<button
					on:click={() => {
						if (tempDomainAdd === '') {
							return;
						}
						appChangeData.domains = [...appChangeData.domains, tempDomainAdd];
					}}
				>
					<svg
						class="w-6 h-6 inline-flex items-center"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 6v6m0 0v6m0-6h6m-6 0H6"
						/>
					</svg>
				</button>
			</div>
			<div class="flex flex-row gap-2">
				{#each appChangeData.domains as domain, index}
					<div class="flex justify-center w-fit">
						<div
							class="mt-2 mr-1 inline-flex items-center rounded bg-indigo-100 text-sm"
						>
							<span class="ml-2 mr-1 max-w-xs truncate leading-relaxed">{domain}</span
							>
							<button
								on:click={() => {
									appChangeData.domains.splice(index);
									appChangeData.domains = appChangeData.domains;
								}}
								class="inline-block h-8 w-6 align-middle text-gray-500 hover:text-gray-600 focus:outline-none"
							>
								<svg
									class="mx-auto h-6 w-6 fill-current"
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
								>
									<path
										fill-rule="evenodd"
										d="M15.78 14.36a1 1 0 0 1-1.42 1.42l-2.82-2.83-2.83 2.83a1 1 0 1 1-1.42-1.42l2.83-2.82L7.3 8.7a1 1 0 0 1 1.42-1.42l2.83 2.83 2.82-2.83a1 1 0 0 1 1.42 1.42l-2.83 2.83 2.83 2.82z"
									/>
								</svg>
							</button>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</div>
	<button
		on:click={saveData}
		class="flex items-center bg-white text-gray-500 p-2 text-sm w-auto hover:bg-green-400 transition bg-gray-200 rounded-md py-2 px-2"
	>
		<!-- Heroicons: outline/save -->
		<svg
			class="w-5 h-5 pr-0.5"
			fill="none"
			stroke="currentColor"
			viewBox="0 0 24 24"
			xmlns="http://www.w3.org/2000/svg"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4"
			/>
		</svg>
		<span>Save</span>
	</button>
{/await}
