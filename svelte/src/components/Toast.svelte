<script lang="ts">
  let {
    message = '',
    type = 'info',
    duration = 3000,
    show = $bindable(false),
  }: {
    message: string;
    type: 'info' | 'error' | 'success' | 'warning';
    duration: number;
    show: boolean;
  } = $props();

  $effect(() => {
    if (show) {
      setTimeout(() => {
        show = false;
      }, duration);
    }
  });

  // Icon SVGs
  const icons = {
    info: `<svg width="20" height="20" fill="none"><circle cx="10" cy="10" r="9" stroke="#2196f3" stroke-width="2"/><rect x="9" y="7" width="2" height="6" rx="1" fill="#2196f3"/><rect x="9" y="5" width="2" height="2" rx="1" fill="#2196f3"/></svg>`,
    success: `<svg width="20" height="20" fill="none"><circle cx="10" cy="10" r="9" stroke="#4caf50" stroke-width="2"/><path d="M6 11l3 3 5-5" stroke="#4caf50" stroke-width="2" fill="none"/></svg>`,
    error: `<svg width="20" height="20" fill="none"><circle cx="10" cy="10" r="9" stroke="#f44336" stroke-width="2"/><path d="M7 7l6 6M13 7l-6 6" stroke="#f44336" stroke-width="2"/></svg>`,
    warning: `<svg width="20" height="20" fill="none"><circle cx="10" cy="10" r="9" stroke="#ff9800" stroke-width="2"/><rect x="9" y="5" width="2" height="7" rx="1" fill="#ff9800"/><rect x="9" y="13" width="2" height="2" rx="1" fill="#ff9800"/></svg>`,
  };
</script>

<div class="toast {type} {show ? 'show' : ''}">
  <span class="icon">{@html icons[type]}</span>
  <span>{message}</span>
</div>

<style>
  .toast {
    display: flex;
    align-items: center;
    min-width: 220px;
    border-radius: 5px;
    padding: 12px 20px;
    transform: translateX(120%);
    z-index: 1000;
    opacity: 0;
    visibility: hidden;
    transition:
      opacity 0.3s,
      visibility 0.3s,
      transform 0.3s;
    font-size: 1rem;
    gap: 12px;
    pointer-events: none;
  }

  .toast.show {
    opacity: 1;
    visibility: visible;
    transform: translateX(0);
    pointer-events: auto;
  }
  .toast.info {
    background: #e3f2fd;
    color: #1565c0;
  }
  .toast.success {
    background: #e8f5e9;
    color: #256029;
  }
  .toast.error {
    background: #ffebee;
    color: #b71c1c;
  }
  .toast.warning {
    background: #fff8e1;
    color: #ff6f00;
  }
  .icon {
    display: flex;
    align-items: center;
  }
</style>
