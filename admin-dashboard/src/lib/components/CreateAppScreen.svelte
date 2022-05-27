<script lang="ts">
    import { validateSchema } from "@felte/validator-yup";
    import { createForm } from "felte";
    import reporter from "@felte/reporter-tippy";
    import * as yup from "yup";
    import "tippy.js/dist/tippy.css";
    import Switch from "./Switch.svelte";

    export let screenSelected: string;

    const registerSchema = yup.object({
        name: yup
            .string()
            .required("Name mustn't be empty")
            .min(4, "Name must be longer than 4 characters."),
        description: yup.string(),
        token_lifetime: yup.string().required(),
        domains: yup.array(yup.string()),
        enforce_totp: yup.boolean()
    });
    let domains = [];
    const {
        form,
        errors,
        touched,
        isValid,
        isSubmitting,
        data,
        validate
    } = createForm<yup.InferType<typeof registerSchema>>({
        validate: [
            validateSchema(registerSchema),
            (values) => {
                const errors = {};
                if (domains.length < 1) {
                    errors.domains = "no domains!";
                }
                return errors;
            }
        ],
        extend: [reporter()],
        initialValues: {
            token_lifetime: "3600",
            enforce_totp: false
        },
        onSubmit: async (values) => {
            const res = await fetch("/api/v1/admin/app", {
                method: "post",
                body: JSON.stringify({
                    name: values.name,
                    description: values.description,
                    token_lifetime: parseInt(values.token_lifetime),
                    domains: domains,
                    enforce_totp: values.enforce_totp
                }),
                headers: {
                    "Content-Type": "application/json"
                }
            });
            screenSelected = "home";
        }
    });

    let tempDomain = "";
</script>

<div class="flex w-screen">
    <button type="button"
            class="bg-white rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500 place-self-end"
            on:click={() => {
		screenSelected = 'home';
	}}>
        <span class="sr-only">Close menu</span>
        <!-- Heroicons: outline/x -->
        <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor"
             aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
    </button>
</div>
<form use:form>
    <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 flex flex-col">
        <div class="mb-4">
            <label class="block text-grey-darker text-sm font-bold mb-2" for="name">
                Name
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-800"
              id="name"
              type="text"
              name="name"
              class:border-red-300={$errors.name !== null}
              class:border-green-400={$touched.name === true && $errors.name === null}
              placeholder="Name"
            />
        </div>
        <div class="mb-4">
            <label class="block text-grey-darker text-sm font-bold mb-2" for="description">
                Description
            </label>
            <textarea
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-800"
              id="description"
              name="description"
              class:border-red-300={$errors.description !== null}
              class:border-green-400={$touched.description === true &&
					$errors.description === null}
              placeholder="Description"
            />
        </div>
        <div class="mb-6">
            <label class="block text-gray-800 text-sm font-bold mb-2" for="token_lifetime">
                Token Lifetime
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-800 mb-3"
              name="token_lifetime"
              class:border-red-300={$errors.token_lifetime !== null}
              class:border-green-400={$touched.token_lifetime === true &&
					$errors.token_lifetime === null}
              id="token_lifetime"
            />
        </div>
        <div class="mb-6">
            <input type="text" bind:value={tempDomain} />
            <button
              type="button"
              on:click={() => {
					domains = [...domains, tempDomain];
					tempDomain = '';
					validate();
				}}
            >
                Add domain
            </button>
            {#each domains as domain, index}
                <div>
                    {domain}
                    <button
                      type="button"
                      on:click={() => {
							domains.splice(index);
							domains = domains;
							validate();
						}}
                    >
                        Remove domain
                    </button>
                </div>
            {/each}
        </div>
        <div class="mb-6">
            <p>Enforce TOTP?</p>
            <Switch bind:on={$data.enforce_totp} />
        </div>

        <div class="flex items-center justify-between">
            <button
              class="bg-blue-600 hover:bg-blue-800 text-white font-bold py-2 px-4 rounded disabled:opacity-50 disabled:cursor-not-allowed"
              type="submit"
              disabled={!$isValid || $isSubmitting}
              class:cursor-not-allowed={!$isValid || $isSubmitting}
              class:opacity-50={!$isValid || $isSubmitting}
            >
                {#if $isSubmitting}
                    <svg class="h-4 w-4 animate-spin" viewBox="3 3 18 18">
                        <path
                          class="fill-blue-800"
                          d="M12 5C8.13401 5 5 8.13401 5 12C5 15.866 8.13401 19 12 19C15.866 19 19 15.866 19 12C19 8.13401 15.866 5 12 5ZM3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12Z"
                        />
                        <path
                          class="fill-blue-100"
                          d="M16.9497 7.05015C14.2161 4.31648 9.78392 4.31648 7.05025 7.05015C6.65973 7.44067 6.02656 7.44067 5.63604 7.05015C5.24551 6.65962 5.24551 6.02646 5.63604 5.63593C9.15076 2.12121 14.8492 2.12121 18.364 5.63593C18.7545 6.02646 18.7545 6.65962 18.364 7.05015C17.9734 7.44067 17.3403 7.44067 16.9497 7.05015Z"
                        />
                    </svg>
                {:else}
                    Create
                {/if}
            </button>
            <!-- <a class="inline-block align-baseline font-bold text-sm text-blue hover:text-blue-800" href="#">
                 Forgot Password?
             </a>-->
        </div>
    </div>
</form>
