<!-- YOU CAN DELETE EVERYTHING IN THIS PAGE -->
<script lang="ts">
  import type { PageData } from "./$types";
  import { invalidateAll } from "$app/navigation";

  export let data: PageData;
  let todos = data.todos;
  
  async function deleteTodo(id: number) {
    await fetch(`3.133.151.30:8000/delete/${id}`);
    invalidateAll();
  }
</script>

<div class="container mx-auto mt-16">
  <h1 class="text-5xl font-bold text-center underline m-10">Todos</h1>
  
  <form action="3.133.151.30:8000/create" method="POST">
    <input class="input my-4" type="text" name="id" placeholder="Enter Todo ID" />
    <input class="input my-4" type="text" name="title" placeholder="Enter Todo Title" />
    <input class="input my-4" type="text" name="description" placeholder="Enter Todo Description" />
    <input class="input my-4" type="text" name="done" placeholder="Enter Todo Status" />
    <button class="btn btn-primary bg-blue-600 h-16 w-40 text-lg" type="submit">Add Todo</button>
  </form>


  <div class="flex justify-center flex-col flex-col-2">
    {#each data.todos as todo}
      <div class="card bg-base-100 shadow-xl p-8 mx-16 my-4 text-xl h-[20rem] w-[36rem]">
        <p class="text-2xl font-bold m-4"><span class="text-secondary-500">ID:</span> {todo.id}</p>
        <p class="text-2xl font-bold m-4"><span class="text-secondary-500">Title:</span> {todo.title}</p>
        <p class="text-2xl font-bold m-4"><span class="text-secondary-500">Description:</span> {todo.description}</p>
        <p class="text-2xl font-bold m-4"><span class="text-secondary-500">Completed:</span> {todo.done}</p>
      </div>
      <div class="flex mb-8">
        <button class="btn mx-16 my-2 variant-filled-primary  h-16 w-40 text-lg" type="submit" on:click={deleteTodo(todo.id)}>Delete Todo</button>
        <button class="btn my-2 variant-filled-tertiary  h-16 w-40 text-lg" type="submit">Update Todo</button>
      </div>
    {/each}
  </div>
</div>
