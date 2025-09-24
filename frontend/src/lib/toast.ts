import { toast, ToastOptions, Id } from 'react-toastify';
import { TxStatus } from '@/lib/types';

// Default toast options
const defaultOptions: ToastOptions = {
    position: "bottom-right",
    autoClose: 5000,
    hideProgressBar: false,
    closeOnClick: true,
    pauseOnHover: true,
    draggable: true,
    progress: undefined,
};

// Create success toast content with transaction hash
const createSuccessContent = (txHash?: string): string => {
    if (txHash) {
        return `Transaction completed: ${txHash}`;
    }
    return "Transaction successful!";
};

// Create error toast content with message and transaction hash
const createErrorContent = (errorMessage?: string, txHash?: string): string => {
    let content = "Transaction failed";

    if (errorMessage) {
        content += `: ${errorMessage}`;
    }

    if (txHash) {
        content += ` (${txHash})`;
    }

    return content;
};

// Toast for transaction status
export const txToast = {
    // Show a loading toast when transaction is being processed
    loading: (message: string = 'Processing transaction...'): Id => {
        return toast.loading(message, defaultOptions);
    },

    // Update toast when transaction status changes
    update: (toastId: Id, status: TxStatus, txHash?: string, errorMessage?: string): void => {
        switch (status) {
            case TxStatus.BUILDING:
                toast.update(toastId, {
                    render: 'Building transaction...',
                    type: "info",
                    isLoading: true,
                });
                break;
            case TxStatus.SIGNING:
                toast.update(toastId, {
                    render: 'Waiting for signature...',
                    type: "info",
                    isLoading: true,
                });
                break;
            case TxStatus.SUBMITTING:
                toast.update(toastId, {
                    render: txHash ? `Transaction submitted: ${txHash}` : 'Submitting transaction...',
                    type: "info",
                    isLoading: true,
                });
                break;
            case TxStatus.SUCCESS:
                toast.update(toastId, {
                    render: createSuccessContent(txHash),
                    type: "success",
                    isLoading: false,
                    autoClose: 7000,
                });
                break;
            case TxStatus.FAIL:
                toast.update(toastId, {
                    render: createErrorContent(errorMessage, txHash),
                    type: "error",
                    isLoading: false,
                    autoClose: 10000,
                });
                break;
            default:
                break;
        }
    },

    // Show success toast
    success: (message: string): Id => {
        return toast.success(message, defaultOptions);
    },

    // Show error toast
    error: (message: string): Id => {
        return toast.error(message, {
            ...defaultOptions,
            autoClose: 7000,
        });
    },

    // Show info toast
    info: (message: string): Id => {
        return toast.info(message, defaultOptions);
    },

    // Dismiss a specific toast
    dismiss: (toastId: Id): void => {
        toast.dismiss(toastId);
    },

    // Dismiss all toasts
    dismissAll: (): void => {
        toast.dismiss();
    }
};