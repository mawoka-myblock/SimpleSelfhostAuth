<script lang="ts">
	import { loggedIn } from '../stores';
	import Spinner from './Spinner.svelte';
	import type { PrivateUser } from '../../app';
	import Switch from './Switch.svelte';
	import { clickOutside } from '../clickOutside';

	export let user_id: string;
	export let screenSelected: string;
	let userChangeData: {
		username: string;
		email: string;
		admin: boolean;
		scopes: Array<string>;
	};

	let user_initial: PrivateUser | null;

	const getUserFunction = async (): Promise<null | PrivateUser> => {
		const res = await fetch(`/api/v1/admin/user?id=${user_id}`);
		if (res.status === 401) {
			loggedIn.set({ status: false, admin: false });
			return;
		}
		if (res.status === 200) {
			const json = await res.json();
			userChangeData = {
				username: json.username,
				admin: json.admin,
				email: json.email,
				scopes: json.scopes
			};
			user_initial = json;
			return json;
		}
	};

	const saveData = async () => {
		let data_to_submit = {
			id: user_id,
			username:
				userChangeData.username === user_initial.username
					? undefined
					: userChangeData.username,
			admin: userChangeData.admin === user_initial.admin ? undefined : userChangeData.admin,
			email: userChangeData.email === user_initial.email ? undefined : userChangeData.email,
			scopes:
				JSON.stringify(userChangeData.scopes) === JSON.stringify(user_initial.scopes)
					? undefined
					: userChangeData.scopes
		};
		const res = await fetch('/api/v1/admin/user', {
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
			getUser = getUserFunction();
		}
	};
	let tempScopeAdd = '';
	let getUser = getUserFunction();
	const deleteUser = async () => {
		if (!delete_confirm) {
			return;
		}
		await fetch(`/api/v1/admin/user?id=${user_id}`, {
			method: 'DELETE'
		});
		screenSelected = 'home';
	};
	let delete_confirm = false;
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
<br />
<button
	class="relative inline-block px-4 py-1 overflow-hidden border border-red-600 group focus:outline-none focus:ring rounded-md"
	on:click={() => {
		deleteUser().then();
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
			Delete User
		{/if}
	</span>
</button>

{#await getUser}
	<Spinner />
{:then user}
	<div class="pl-2 pb-4">
		<div>
			<label for="username">Username: </label>
			<input
				id="username"
				type="text"
				bind:value={userChangeData.username}
				class="border-b border-dotted border-black"
			/>
		</div>
		<div>
			<label for="email">Email: </label>
			<input
				id="email"
				type="email"
				bind:value={userChangeData.email}
				class="border-b border-dotted border-black"
			/>
		</div>
		<div>
			<label>Admin: </label>
			<Switch bind:on={userChangeData.admin} />
		</div>
		<div>
			<div>
				<label for="addScope">Add scope:</label>
				<input
					id="addScope"
					type="text"
					bind:value={tempScopeAdd}
					class="border-b border-dotted border-black"
				/>
				<button
					on:click={() => {
						if (tempScopeAdd === '') {
							return;
						}
						userChangeData.scopes = [...userChangeData.scopes, tempScopeAdd];
						tempScopeAdd = '';
					}}
				>
					<span class="w-fit">
						<!-- Heroicons: outline/plus -->
						<svg
							class="w-6 h-6 inline-flex items-center"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							xmlns="http://www.w3.org/2000/svg"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 6v6m0 0v6m0-6h6m-6 0H6"
							/></svg
						>
					</span>
				</button>
			</div>
			<div class="flex flex-row gap-2">
				{#each userChangeData.scopes as scope, index}
					<div class="flex justify-center w-fit">
						<div
							class="mt-2 mr-1 inline-flex items-center rounded bg-indigo-100 text-sm"
						>
							<span class="ml-2 mr-1 max-w-xs truncate leading-relaxed">{scope}</span>
							<button
								on:click={() => {
									userChangeData.scopes.splice(index);
									userChangeData.scopes = userChangeData.scopes;
								}}
								class="inline-block h-8 w-6 align-middle text-gray-500 hover:text-gray-600 focus:outline-none"
							>
								<!-- Heroicons: outline/x -->

								<svg
									class="mx-auto h-6 w-6 fill-current"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
									xmlns="http://www.w3.org/2000/svg"
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
