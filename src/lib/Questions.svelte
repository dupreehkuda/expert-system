<script>
    import { invoke } from '@tauri-apps/api/tauri'

    $: question = ''
    $: decision = ''
    $: started = false

    async function process(answer) {
        if (answer === "start") {
            started = true
        }

        let response = await invoke('process_answer', { answer })

        console.log(response)

        question = response.question
        decision = response.decision
    }
</script>

<div>
    <h3>{question}</h3>
    <span>{decision}</span>
    {#if !started}
        <button on:click={async () => process("start")}>Start</button>
    {:else }
        <button on:click={async () => process("yes")}>yes</button>
        <button on:click={async () => process("no")}>no</button>
    {/if}
</div>

<style>
    h3 {
        color: crimson;
    }
</style>