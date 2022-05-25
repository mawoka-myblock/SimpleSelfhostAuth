<script lang="ts">
    import { loggedIn } from "../stores";
    import Spinner from "./Spinner.svelte";
    import type { App } from "../../app";
    import { clickOutside } from "../clickOutside";

    export let app_id: string;
    export let screenSelected: string;
    let appChangeData: {
        name: string;
        description: string;
        token_lifetime: number;
        domains: Array<string>;
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
                token_lifetime: json.token_lifetime
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
                    : appChangeData.domains
        };
        const res = await fetch("/api/v1/admin/app", {
            method: "PATCH",
            headers: {
                "Content-Type": "application/json"
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
    let tempDomainAdd = "";
    let getApp = getAppFunction();
    let delete_confirm = false;

    const deleteApp = async () => {
        if (!delete_confirm) {
            return;
        }
        await fetch(`/api/v1/admin/app?id=${app_id}`, {
            method: "DELETE"
        });
        delete_confirm = false;
        screenSelected = "home";
    };
</script>

<button
  on:click={() => {
		screenSelected = 'home';
	}}>Close
</button
>
<br />

<button
  class="relative inline-block px-8 py-3 overflow-hidden border border-red-600 group focus:outline-none focus:ring"
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
    <div>
        <div>
            <label for="name">Appname: </label>
            <input id="name" type="text" bind:value={appChangeData.name} />
        </div>
        <div>
            <label for="description">Description: </label>
            <textarea id="description" type="text" bind:value={appChangeData.description} />
        </div>
        <div>
            <label for="token_lifetime">Token Lifetime: </label>
            <input id="token_lifetime" type="number" bind:value={appChangeData.token_lifetime} />
        </div>
        <div>
            <div>
                <label for="addDomain">Add domain:</label>
                <input id="addDomain" type="text" bind:value={tempDomainAdd} />
                <button
                  on:click={() => {
						appChangeData.domains = [...appChangeData.domains, tempDomainAdd];
					}}
                >Add domain
                </button>
            </div>
            <div class="flex flex-row gap-2">
                {#each appChangeData.domains as domain, index}
                    <div class="flex justify-center w-fit">
                        <div
                          class="mt-2 mr-1 inline-flex items-center rounded bg-indigo-100 text-sm"
                        >
                            <span class="ml-2 mr-1 max-w-xs truncate leading-relaxed">{domain}</span>
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
    <button on:click={saveData}>Save</button>
{/await}
