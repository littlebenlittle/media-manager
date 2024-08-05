package protocol

type ID struct {
	Repr string
}

type Data struct {
	ID       string
	Resolver string
	Data     []byte
}

type Association struct {
	Data     ID
	Metadata ID
}

// Protocol:
// 1. Exactly 1 field must be non-null (impl: TODO)
type Event struct {
	Register    *Data
	Associate   *Association
	Forget      *Data
	Subscribe   *string
	Unsubscribe *string
}

func (e Event) Validate() []string {
	// TODO
	return nil
}

type ServerMsg struct {
	Errors []string
	Event  *Event
	// ID of accepted client message
	Accepted *string
}

type ClientMsg struct {
	Errors []string
	Event  Event
	ID     string
}

const (
	RESOLVER_BASE64_URL = "base64_url"
)
