<template>
  <div class="empty-content">
    <img src="@src/assets/icons/svg/common/no_data.svg" class="empty-img" />
    <div class="empty-text">
      <span>{{ text }}</span>
      <slot></slot>
    </div>
  </div>
</template>

<script setup>
defineProps({
  text: {
    type: String,
    default: ''
  },
  showContaner: {
    type: Boolean,
    default: true
  }
})
</script>

<style lang="scss" scoped>
.empty-content {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  .empty-img {
    width: 270px;
    height: 180px;
    object-fit: contain;
  }
  .empty-text {
    display: flex;
    gap: 16px;
    align-items: center;
    color: rgb(52, 52, 52);
    font-size: 20px;
    font-weight: 400;
  }
}
</style>
