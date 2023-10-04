<script>
    import { invoke } from '@tauri-apps/api/tauri'

    $: question = ''
    $: decision = ''
    $: started = false
    $: finished = false
    $: possibleAnswers = []
    $: logQuestions = []
    $: logAnswers = []

    async function addToLog(answer) {
        if (answer === "start") { return }

        logQuestions = [...logQuestions, question]
        logAnswers = [...logAnswers, answer]
    }

    async function process(answer) {
        if (answer === "start") { started = true }

        await addToLog(answer)

        let response = await invoke("process_answer", { answer })

        question = response.question
        decision = response.decision
        possibleAnswers = response.answers

        if (decision !== "") { finished = true } else { finished = false }
    }

    async function restart() {
        question = ''
        decision = ''
        logQuestions = []
        logAnswers = []
        possibleAnswers = []

        await invoke("reset_node")
        await process("start")
    }
</script>

<div class="flex flex-col justify-center items-center h-screen header">
    <h2 class="py-2">{question}</h2>
    <h2 class="py-2">{decision}</h2>

    <div class="flex py-2">
        {#if !started && !finished}
            <button class="btn mr-2" on:click={async () => process("start")}>Начать</button>
        {:else if started && finished}
            <button class="btn mr-2" on:click={async () => restart()}>В начало</button>
        {:else}
            {#each possibleAnswers as ans}
                <button class="btn mr-2" on:click={async () => process(ans)}>{ans}</button>
            {/each}
        {/if}
    </div>
    <div class="flex py-2">
        <ol>
            {#each logQuestions as question, i}
                <li><h6>{question} -> <span class="answer">{logAnswers[i]}</span></h6></li>
            {/each}
        </ol>
    </div>
</div>

<style>
    .answer {
        font-style: italic;
        color: dodgerblue;
    }
</style>