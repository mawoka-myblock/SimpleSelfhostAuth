<script lang="ts">
	import { validateSchema } from '@felte/validator-yup';
	import { createForm } from 'felte';
	import reporter from '@felte/reporter-tippy';
	import * as yup from 'yup';
	import 'tippy.js/dist/tippy.css';

	export let screenSelected: string;

	const registerSchema = yup.object({
		username: yup
			.string()
			.required("Username mustn't be empty")
			.min(4, 'Username must be longer than 4 characters.'),
		email: yup.string().email('Email must be a valid email').required("Email can't be empty"),
		password: yup
			.string()
			.required("Password mustn't be empty")
			.min(8, 'Password must be longer than 8 characters.'),
		password2: yup
			.string()
			.required()
			.test('equal', 'Passwords do not match!', function (v) {
				const ref = yup.ref('password');
				return v === this.resolve(ref);
			})
	});
	const { form, errors, touched, isValid, isSubmitting } = createForm<
		yup.InferType<typeof registerSchema>
	>({
		validate: validateSchema(registerSchema),
		extend: [reporter()],
		onSubmit: async (values) => {
			await fetch('/api/v1/users/create', {
				method: 'post',
				body: JSON.stringify({
					email: values.email,
					password: values.password,
					username: values.username
				}),
				headers: {
					'Content-Type': 'application/json'
				}
			});
			screenSelected = 'home';
		}
	});
</script>

<button
	on:click={() => {
		screenSelected = 'home';
	}}>Close</button
>
<form use:form>
	<div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 flex flex-col">
		<div class="mb-4">
			<label class="block text-grey-darker text-sm font-bold mb-2" for="username">
				Username
			</label>
			<input
				class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-800"
				id="username"
				type="text"
				name="username"
				class:border-red-300={$errors.username !== null}
				class:border-green-400={$touched.username === true && $errors.username === null}
				placeholder="Username"
			/>
		</div>
		<div class="mb-4">
			<label class="block text-grey-darker text-sm font-bold mb-2" for="email"> Email </label>
			<input
				class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-800"
				id="email"
				type="email"
				name="email"
				class:border-red-300={$errors.email !== null}
				class:border-green-400={$touched.email === true && $errors.email === null}
				placeholder="Username"
			/>
		</div>
		<div class="mb-6">
			<label class="block text-gray-800 text-sm font-bold mb-2" for="password">
				Password
			</label>
			<input
				class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-800 mb-3"
				name="password"
				class:border-red-300={$errors.password !== null}
				class:border-green-400={$touched.password === true && $errors.password === null}
				id="password"
				type="password"
				placeholder="******************"
			/>
			<p class="text-red text-xs italic">Please choose a password.</p>
		</div>
		<div class="mb-6">
			<label class="block text-gray-800 text-sm font-bold mb-2" for="password2">
				Repeat Password
			</label>
			<input
				class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-800 mb-3"
				name="password2"
				class:border-red-300={$errors.password2 !== null}
				class:border-green-400={$touched.password2 === true && $errors.password2 === null}
				id="password2"
				type="password"
				placeholder="******************"
			/>
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
					Register
				{/if}
			</button>
			<!-- <a class="inline-block align-baseline font-bold text-sm text-blue hover:text-blue-800" href="#">
                 Forgot Password?
             </a>-->
		</div>
	</div>
</form>
