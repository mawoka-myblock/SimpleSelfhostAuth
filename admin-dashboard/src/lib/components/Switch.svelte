<script lang="ts">
	export let on: boolean;
</script>

{#if on}
	<button
		type="button"
		role="switch"
		aria-checked="true"
		id="admin"
		class="relative inline-flex h-5 w-8 shrink-0 cursor-pointer appearance-none rounded-full border-2 border-transparent bg-blue-700 transition focus:outline-none focus:ring focus:ring-blue-200"
		on:click={() => (on = !on)}
	>
		<span
			aria-hidden="true"
			class="pointer-events-none flex h-4 w-4 translate-x-3 items-center justify-center rounded-full bg-white transition will-change-transform"
		>
			<svg class="h-2.5 w-2.5 shrink-0 stroke-blue-700" viewBox="0 0 256 256">
				<polyline
					points="216 72 104 184 48 128"
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="36"
					fill="none"
				/>
			</svg>
		</span>
	</button>
{:else}
	<button
		on:click={() => (on = !on)}
		type="button"
		role="switch"
		aria-checked="false"
		class="relative inline-flex h-5 w-8 shrink-0 cursor-pointer appearance-none rounded-full border-2 border-transparent bg-gray-200 transition focus:outline-none focus:ring focus:ring-blue-200"
	>
		<span
			aria-hidden="true"
			class="pointer-events-none flex h-4 w-4 translate-x-0 items-center justify-center rounded-full bg-white transition will-change-transform"
		>
			<svg class="h-2.5 w-2.5 shrink-0 stroke-gray-600" viewBox="0 0 256 256">
				<line
					x1="200"
					y1="56"
					x2="56"
					y2="200"
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="36"
				/>
				<line
					x1="200"
					y1="200"
					x2="56"
					y2="56"
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="36"
				/>
			</svg>
		</span>
	</button>
{/if}
