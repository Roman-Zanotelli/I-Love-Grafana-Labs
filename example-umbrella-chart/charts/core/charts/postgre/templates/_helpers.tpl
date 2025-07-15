{{- define "postgres.fullname" -}}
{{ .Release.Name }}-{{ .name }}
{{- end }}