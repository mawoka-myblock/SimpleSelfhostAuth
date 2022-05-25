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

<button
	on:click={() => {
		screenSelected = 'home';
	}}
	>Close
</button>
<br />
<button
	class="relative inline-block px-8 py-3 overflow-hidden border border-red-600 group focus:outline-none focus:ring"
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
	<div>
		<div>
			<label for="username">Username: </label>
			<input id="username" type="text" bind:value={userChangeData.username} />
		</div>
		<div>
			<label for="email">Email: </label>
			<input id="email" type="email" bind:value={userChangeData.email} />
		</div>
		<div>
			<label>Admin: </label>
			<Switch bind:on={userChangeData.admin} />
		</div>
		<div>
			<div>
				<label for="addScope">Add scope:</label>
				<input id="addScope" type="text" bind:value={tempScopeAdd} />
				<button
					on:click={() => {
						if (tempScopeAdd === '') {
							return;
						}
						userChangeData.scopes = [...userChangeData.scopes, tempScopeAdd];
						tempScopeAdd = '';
					}}
					>Add scope
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
	<button on:click={saveData}>Save</button>
{/await}
