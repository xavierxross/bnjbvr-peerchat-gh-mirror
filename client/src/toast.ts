import { toast } from "@zerodevx/svelte-toast";

interface ToastOptions {
    duration?: number;
}

export default {
    info(msg: String, opts?: ToastOptions) {
        toast.push(msg, opts)
    },

    error(msg: String) {
        toast.push(msg, {
            theme: {
                "--toastBackground": "#F56565",
                "--toastProgressBackground": "#C53030",
            },
        });
    }
}
