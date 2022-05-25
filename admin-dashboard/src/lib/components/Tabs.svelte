<script lang="ts">
	export let available_tabs: Array<string>;
	export let selected_tab: string;
</script>

<div class="flex border-b border-gray-200 w-screen">
	{#each available_tabs as tab, index}
		{#if tab === selected_tab}
			<button
				class="-mb-px h-10 whitespace-nowrap border-b-4 border-green-500 bg-transparent px-4 py-2 text-center text-sm text-green-500 focus:outline-none sm:text-base grow font-bold"
			>
				{tab}
			</button>
		{:else}
			<button
				class="-mb-px h-10 whitespace-nowrap border-b-2 cursor-base bg-transparent px-4 py-2 text-center text-sm focus:outline-none sm:text-base border-transparent text-gray-700 hover:border-gray-400 grow"
				on:click={() => {
					selected_tab = tab;
				}}
			>
				{tab}
			</button>
		{/if}
	{/each}
</div>
