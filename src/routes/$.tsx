import { Header } from '@/components/ui/header';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/$')({
  component: FourOFour,
})

function FourOFour() {
  return (
    <div className="p-2">
      <img
        src="/hsr/icon/avatar/202006.png"
      />
      <Header level={1}>How the fuck did you manage to 404 a native application</Header>
    </div>
  )
}
