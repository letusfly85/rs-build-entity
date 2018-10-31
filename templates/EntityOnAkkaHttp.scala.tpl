package entity

{% set i = 0 -%}
{% set suffix = "," -%}
final case class {{ EntityName }} (
{%for column in column_list -%}
  {% set i = i + 1 -%}
  {%if i >= column_list_length -%}
  {% set suffix = "" -%}
  {%endif -%}
  {%if column.is_nullable == "YES" -%}
    {{ column.column_name_camel }}: Option[{{ column.data_type }}]{{ suffix }}
  {%else -%}
    {{ column.column_name_camel }}: {{ column.data_type }}{{ suffix }}
  {%endif -%}
{% endfor -%}) {
{%for column in column_list -%}
  {%if column.data_type == "Int" -%}
    require({{ column.column_name_camel }}.nonEmpty, "")
  {%else -%}
    require({{ column.column_name_camel }}, "")
  {%endif -%}
{% endfor -%}
}
