<script setup lang="ts">
import { ListTodo, X, Loader2 } from "lucide-vue-next";
import { useStudio } from "../../composables/useStudio";
import { useI18n } from "../../i18n";
import type { TaskStatus } from "../../types";

const studio = useStudio();
const { t } = useI18n();

const statusClass: Record<TaskStatus, string> = {
  pending: "",
  running: "badge-accent",
  succeeded: "badge-success",
  failed: "badge-danger",
  cancelled: "",
  interrupted: "",
};

function excerpt(prompt: string): string {
  const trimmed = prompt.trim();
  return trimmed.length > 90 ? `${trimmed.slice(0, 90)}…` : trimmed;
}

function timeLabel(iso: string): string {
  if (!iso) return "";
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) return iso;
  return date.toLocaleString(undefined, {
    month: "numeric",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}
</script>
<template>
  <section class="settings-section">
    <h2>{{ t("tasks.title") }}</h2>
    <p>{{ t("tasks.sub") }}</p>
    <div v-if="!studio.state.tasks.length" class="task-empty">
      <ListTodo :size="22" /><span>{{ t("tasks.empty") }}</span>
    </div>
    <ul v-else class="task-list">
      <li v-for="task in studio.state.tasks" :key="task.id">
        <span class="badge" :class="statusClass[task.status]"
          ><Loader2
            v-if="task.status === 'running'"
            :size="10"
            class="spin" />{{ t(`tasks.status.${task.status}`) }}</span
        >
        <div class="task-main">
          <span class="task-prompt">{{ excerpt(task.prompt) }}</span
          ><span class="task-meta"
            >{{ task.model }} · {{ timeLabel(task.createdAt) }}<template
              v-if="task.status === 'succeeded' && task.resultCount"
              > · {{ t("tasks.results", { n: task.resultCount }) }}</template
            ></span
          >
        </div>
        <button
          v-if="task.status === 'pending' || task.status === 'running'"
          type="button"
          class="icon-btn"
          :aria-label="t('tasks.cancel')"
          :data-tip="t('tasks.cancel')"
          @click="studio.cancelTask(task.id)"
        >
          <X :size="14" />
        </button>
      </li>
    </ul>
  </section>
</template>
<style scoped>
.task-empty {
  display: flex;
  align-items: center;
  gap: 10px;
  color: var(--text-muted);
  font-size: 11px;
  border-top: 1px solid var(--border);
  padding-top: 24px;
  margin-top: 27px;
}
.task-list {
  list-style: none;
  margin: 27px 0 0;
  padding: 0;
  border-top: 1px solid var(--border);
}
.task-list li {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 13px 0;
  border-bottom: 1px solid var(--border);
}
.task-main {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 3px;
}
.task-prompt {
  font-size: 11px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.task-meta {
  font-size: 9px;
  color: var(--text-muted);
}
.spin {
  animation: task-spin 1.1s linear infinite;
}
@keyframes task-spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
