<template>
  <div class="plugin-config" v-if="config">
    <div v-for="c in config" :key="c.key">
      <div>
        <label>{{ c.key }}</label>
        <input
          type="number"
          :placeholder="c.key"
          v-model.number="c.value"
          @change="updateConfig()"
        />
      </div>
    </div>
  </div>
</template>
