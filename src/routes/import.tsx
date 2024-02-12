import { commands } from "@/bindings.gen";
import { Alert, AlertTitle, AlertDescription } from "@/components/ui/alert";
import { Input } from "@/components/ui/input";
import { Column } from "@/components/util/flex";
import { createFileRoute } from "@tanstack/react-router";
import { AlertCircle } from "lucide-react";
import { useMemo, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { Button } from "@/components/ui/button";
import { useRelics } from "@/store";

export const Route = createFileRoute("/import")({
    component: Import,
});

function Import() {
    const [file, setFile] = useState<File | null>(null);

    const { data, error } = useQuery({
        queryKey: ["parseKelz", file?.name],
        queryFn: async () => {
            if (!file) {
                return null;
            }

            const text = await file.text();
            return commands.parseKelz(text);
        },
    });

    const importedRelics = useMemo(() => {
        if (!data) {
            return undefined;
        }

        if (data.status === "ok") {
            return data.data;
        }

        return undefined;
    }, [data]);

    const errorMessage = useMemo(() => {
        if (!error) {
            return data?.status === "error" ? data.error : undefined;
        }

        return error.message;
    }, [error]);

    const setRelics = useRelics(d => d.setRelics);

    return (
        <Column>
            <Input raw type="file"
                onChange={(e) => {
                    const file = e.target.files?.[0];
                    if (file) {
                        setFile(file);
                    }
                }}
            />

            { errorMessage &&
                <Alert variant="destructive">
                    <AlertCircle className="h-4 w-4" />
                    <AlertTitle>Error</AlertTitle>
                    <AlertDescription>
                        {errorMessage}
                    </AlertDescription>
                </Alert>
            }

            <Button disabled={!importedRelics} onClick={() => {
                if (importedRelics) {
                    setRelics(importedRelics);
                }
            }}>
                Import Relics
            </Button>
        </Column>
    );
}
