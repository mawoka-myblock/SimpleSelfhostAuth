<script lang="ts">
    import { loggedIn } from "../stores";

    let loginData = {
        username: "",
        password: "",
        stay_logged_in: false,
        totp: ""
    };


    const pureLoginFunction = async (totp_token: number | undefined) => {
        const res = await fetch("/api/v1/users/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                username: loginData.username,
                password: loginData.password,
                stay_logged_in: loginData.stay_logged_in,
                totp: totp_token
            })
        });
        if (res.status === 200) {
            const json = await res.json();
            loggedIn.set({ status: true, admin: json.admin });
        } else {
            alert("Login didn't work!");
        }
    };

    const submitLogin = async () => {
        const res = await fetch("/api/v1/users/request_token", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ username: loginData.username })
        });
        if (res.status === 401) {
            await pureLoginFunction(loginData.totp == "" ? undefined : parseInt(loginData.totp));
        } else if (res.status === 200) {
            const totp_token = prompt("A token has been sent to your device. Please enter it.");
            await pureLoginFunction(parseInt(totp_token));
        } else {
            alert("Unexpected error!")
        }


    };
</script>

<div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 flex flex-col">
    <div class="mb-4">
        <label class="block text-grey-darker text-sm font-bold mb-2" for="username">
            Username
        </label>
        <input
          class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-800"
          id="username"
          type="text"
          placeholder="Username"
          bind:value={loginData.username}
        />
    </div>
    <div class="mb-6">
        <label class="block text-gray-800 text-sm font-bold mb-2" for="password"> Password </label>
        <input
          class="shadow appearance-none rounded w-full py-2 px-3 text-gray-800 mb-3"
          id="password"
          type="password"
          placeholder="******************"
          bind:value={loginData.password}
        />
        <p class="text-red text-xs italic">Please choose a password.</p>
    </div>
    <div class="mb-6">
        <label class="block text-gray-800 text-sm font-bold mb-2" for="rememberme">
            Stay logged in?
        </label>
        <input
          class="shadow border rounded py-2 px-3 text-gray-800 mb-3"
          id="rememberme"
          type="checkbox"
          bind:checked={loginData.stay_logged_in}
        />
    </div>
    <div class="mb-6">
        <label class="block text-gray-800 text-sm font-bold mb-2" for="totp"> TOTP </label>
        <input
          class="shadow border rounded py-2 px-3 text-gray-800 mb-3"
          id="totp"
          type="number"
          max="999999"
          bind:value={loginData.totp}
        />
    </div>
    <div class="flex items-center justify-between">
        <button
          class="bg-blue-600 hover:bg-blue-800 text-white font-bold py-2 px-4 rounded"
          type="button"
          on:click={submitLogin}
        >
            Sign In
        </button>
        <!-- <a class="inline-block align-baseline font-bold text-sm text-blue hover:text-blue-800" href="#">
             Forgot Password?
         </a>-->
    </div>
</div>
